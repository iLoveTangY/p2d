async function main() {
    const wasm = await import("p2d");

    const canvas = document.querySelector(".scene");
    const ctx = canvas.getContext("2d");
    canvas.width = 480;
    canvas.height = 800;

    const world = wasm.P2DWorld.new(1 / 60, 400, 4.0);
    console.log('world: ', world);

    addBorder();

    function renderBall(position, radius) {
        ctx.beginPath();
        ctx.fillStyle = "rgb(110, 123, 108)";
        ctx.arc(position.x, position.y, radius, 0, Math.PI * 2);
        ctx.fill();

    }

    function renderAABB(min, max) {
        ctx.beginPath();
        ctx.fillStyle = "rgb(110, 123, 108)";
        ctx.fillRect(min.x, min.y, max.x, max.y);
    }

    const renderLoop = () => {
        ctx.fillStyle = "rgb(45, 64, 108)";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        world.step();
        for (const body of world.get_bodies()) {
            const position = body.get_position();
            if (body.get_shape_type() === wasm.P2DShapeType.Circle) {
                const circle = body.get_circle();
                renderBall(position, circle.radius);
            } else if (body.get_shape_type() === wasm.P2DShapeType.AABB) {
                const aabb = body.get_aabb();
                renderAABB(wasm.Vec2.new(position.x - aabb.max.x / 2, position.y - aabb.max.y / 2), 
                           wasm.Vec2.new(position.x + aabb.max.x / 2, position.y + aabb.max.y / 2));
            }
        }
        requestAnimationFrame(renderLoop);
    };

    renderLoop();

    canvas.addEventListener("mousedown", (e) => {
        if (e.button === 0) {
            // 左键被按下，在按下的位置生成一个Ball
            const rect = canvas.getBoundingClientRect();
            const ballPosition = wasm.Vec2.new(e.clientX - rect.left, e.clientY - rect.top);
            world.add_body(wasm.P2DBody.new_circle(30, ballPosition, 1.0));
        } else if (e.button === 2) {
            // 右键被按下，在按下的位置生成一个AABB
            const rect = canvas.getBoundingClientRect();
            const pos = wasm.Vec2.new(e.clientX - rect.left, e.clientY - rect.top);
            world.add_body(wasm.P2DBody.new_aabb(wasm.Vec2.new(0, 0), wasm.Vec2.new(60, 60), pos, 1.0));
        }
    });

    function addBorder() {
        const bottomHeight = 20;
        const position = wasm.Vec2.new(canvas.width / 2, canvas.height - bottomHeight / 2);
        const ground = wasm.P2DBody.new_aabb(wasm.Vec2.new(0, 0), wasm.Vec2.new(canvas.width, bottomHeight), position, 0.5);
        ground.make_static();
        world.add_body(ground);
    }
}

main()
