const rust = import('./pkg/rust_3d_demo');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext("webgl", { antialias: true });

rust.then(m => {
    if (!gl) {
        alert('Failed to initialize WebGL');
        return;
    }
    
    const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
    const dougsClient = new m.DougsClient();
    const initialTime = Date.now();
    let lastDrawTime = -1;// In milliseconds

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currTime - initialTime;
            dougsClient.update(elapsedTime, window.innerHeight, window.innerWidth);
            dougsClient.render();
        }
    }

    render();
});
