# 碰撞检测

## Axis Aligned Bounding Boxes（轴对称包围盒）

简称AABB，四条边和坐标系平行，意味着box不能旋转，可以用如下的方式定义

```rust
struct AABB {
    Vec2 min;
    Vec2 max;
}
```

快速的判断两个AABB是否碰撞代码：

```rust
fn AABBvsAABB(a: &AABB, b: &AABB) -> Bool {
    if a.max.x < b.min.x || a.min.x > b.max.x {
        false
    }
    if (a.max < b.min.y || a.min.y > b.max.y) {
        false
    }

    true
}
```

# 碰撞求解——基于冲量（Impulse）

冲量在物理中是一个过程量，表示在一段时间内物体的动量变化。
$$V'=V+\Delta V$$

## 碰撞求解

假设我们已经检测到碰撞，并且已经获得了下面两个重要的信息：

* 碰撞法线（normal）
* 碰撞的侵入量（penetration）

公式一：
$$V^{AB} = V^B - V^A$$

公式二：
$$V^{AB} \cdot n = (V^B - V^A) \cdot n$$

公式三：
$$V_1 = \begin{bmatrix}x_1 \\y_1\end{bmatrix}, V_2 = \begin{bmatrix}x_2 \\y_2\end{bmatrix} \\ V_1 \cdot V_2 = x_1 * x_2 + y_2 * y_2$$

引入恢复系数$e = min(A.e, B.e)$

公式四：
$$V' = e * V$$

公式五：
$$V^{AB} \cdot n = -e * (V^B - V^A) \cdot n$$

公式六：
$$V' = V + j * n$$

公式七：
$$ Impulse = mass * Velocity \\ Velocity = \frac{Impulse}{mass} \therefore V' = V + \frac{j * n}{mass}$$

公式八：
$$V'^A = V^A + \frac{j * n}{mass^A} \\ V'^B = V^B - \frac{j * n}{mass^B}$$

公式九：
$$(V^A - V^V + \frac{j * n}{mass^A} + \frac{j * n}{mass^B}) * n = -e * (V^B - V^A) \cdot n \\ \therefore \\ (V^A - V^V + \frac{j * n}{mass^A} + \frac{j * n}{mass^B}) * n + e * (V^B - V^A) \cdot n = 0$$

公式十：
$$(V^B - V^A) \cdot n + j * (\frac{j * n}{mass^A} + \frac{j * n}{mass^B}) * n + e * (V^B - V^A) \cdot n = 0 \\ \therefore \\ (1 + e)((V^B - V^A) \cdot n) + j * (\frac{j * n}{mass^A} + \frac{j * n}{mass^B}) * n = 0 \\ \therefore \\ j = \frac{-(1 + e)((V^B - V^A) \cdot n)}{\frac{1}{mass^A} + \frac{1}{mass^B}}$$

$j$已经解出来了，那么可以实现碰撞的冲量求解了：

```rust
fn resolveCollision(a: &mut Object, b: &mut Object) {
    let rv: Vec2 = b.velocity - a.velocity;

    let vel_along_normal = rv * normal;

    if vel_along_normal > 0 {
        return;
    }

    let e: float = min(a.restitution, b.restitution);

    let mut j: float = -(1 + e) * vel_along_normal;
    j /= 1 / a.mass + 1 / b.mass;

    let impulse: Vec2 = j * normal;
    a.velocity -= 1 / a.mass * impulse;
    b.velocity += 1 / b.mass * impulse;
}
```

# 模块设计

## Bodise

存储物体所代表的形状、质量数据、变换（位置、旋转）、速度、扭矩等。

```rust
struct Body {
    shape: &Shape,
    tx: Transform,
    material: Material,
    mass_data: MassData,
    velocity: Vec2,
    force: Vec2,
    gravity_scale: f32,
}
```

`gravity_scale`用来缩放物体的重力。

```rust
struct MassData {
    mass: f32,
    inv_mass: f32,

    // 旋转使用
    inertia: f32,
    inverse_inertia: f32,
}
```

质量很不直观，手动设置需要花大量的时间来调试，因此我们会使用如下的公式来定义质量：
$$Mass=density*volume$$

当我们想要调整质量时，我们应该调整密度（density）。乘上体积（volume）之后我们就得到了质量。

## Materials

```rust
struct Material {
    density: f32,
    restitution: f32,
}
```

一旦设置了`material`之后，应该将`material`传递给`shape`来计算`mass`。

一些常见对象的材质信息：

```txt
Rock       Density : 0.6  Restitution : 0.1
Wood       Density : 0.3  Restitution : 0.2
Metal      Density : 1.2  Restitution : 0.05
BouncyBall Density : 0.3  Restitution : 0.8
SuperBall  Density : 0.3  Restitution : 0.95
Pillow     Density : 0.1  Restitution : 0.2
Static     Density : 0.0  Restitution : 0.4
```

## Forces

每次物理系统更新时，`force`的值都是从零开始。物理系统中body收到的影响将会表现为力（force），会被加到force上，然后在integration阶段会用来计算物体的加速度，在integration之后force会置零。

# Broad Phase

前面提到的碰撞检测的过程通常被称为"narrow phase"，其实在 narrow phase之前还有一个阶段通常被称为"broad phase"，broad phase 的主要目的时判断哪些物体可能会发生碰撞，而 narrow phase的目的是判断这些物体到底有没有发生碰撞。

```rust
struct Pair {
    a: &Body,
    b: &Body,
}
```
Broad Phase通常会将可能会发生碰撞的对象两两醉成一个Pair发送给物理引擎的下一个阶段，也就是Narrow phase。

最简单的的碰撞检测算法就是两两检测 Body 的 AABB 是否发生碰撞。注意不要忘记剔除重复项。

更加快速的算法，空间四叉树。

# 分层（Layering）

分层的意义在于可以通过配置层级信息不同层的对象永远不会发生碰撞。
分层最好使用bitmasks来实现。分层应该在broad phase完成。

# Halfspace intersection

halfspace 可以认为是2维空间中一条线的一侧。在物理引擎中检测一个点是否位于一条线的一侧或另一侧是一项非常常见的任务。有一个非常简单的方式来完成这个任务。
直线的一般式方程如下：
$$ax+by+c=0$$
那么直线的法线（normal）就是$\begin{bmatrix}a \\b\end{bmatrix}$
判断一点$(x, y)$是否位于直线的某一侧，只需要将点带入到直线方程中，然后检查结果的符号即可。如果结果是0表示点位于直线上，正号和负号表示分别位于直线的两侧。
