export type Rect = {
    x: number;
    y: number;
    width: number;
    height: number;
};

export class ROISelector {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;

    public rect: Rect = { x: 0, y: 0, width: 0, height: 0 };

    private drawing = false;
    private startX = 0;
    private startY = 0;

    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        this.ctx = canvas.getContext("2d")!;
        this.bindEvents();
    }

    private bindEvents() {
        this.canvas.addEventListener("mousedown", this.onDown);
        this.canvas.addEventListener("mousemove", this.onMove);
        this.canvas.addEventListener("mouseup", this.onUp);
    }

    private onDown = (e: MouseEvent) => {
        this.drawing = true;
        this.startX = e.offsetX;
        this.startY = e.offsetY;
    };

    private onMove = (e: MouseEvent) => {
        if (!this.drawing) return;

        const x = e.offsetX;
        const y = e.offsetY;

        this.rect.x = Math.min(this.startX, x);
        this.rect.y = Math.min(this.startY, y);
        this.rect.width = Math.abs(x - this.startX);
        this.rect.height = Math.abs(y - this.startY);
    };

    private onUp = () => {
        this.drawing = false;
        console.log("ROI selected:", this.rect);
    };

    /** ⭐ 每帧调用 */
    public draw() {
        const r = this.rect;

        if (r.width === 0 || r.height === 0) return;

        const ctx = this.ctx;

        // ======================
        // 半透明填充（关键）
        // ======================
        ctx.fillStyle = "rgba(255, 255, 0, 0.15)";
        ctx.fillRect(r.x, r.y, r.width, r.height);

        // ======================
        // 高亮边框
        // ======================
        ctx.strokeStyle = "yellow";
        ctx.lineWidth = 2;

        ctx.beginPath();
        ctx.rect(r.x, r.y, r.width, r.height);
        ctx.stroke();
    }

    public destroy() {
        this.canvas.removeEventListener("mousedown", this.onDown);
        this.canvas.removeEventListener("mousemove", this.onMove);
        this.canvas.removeEventListener("mouseup", this.onUp);
    }
}