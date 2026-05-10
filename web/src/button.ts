const btn_tpl =`
<div id="control-btn">
<button class="left">
⟲ Left
</button>
<button class="right">
⟳ Right
</button>
</div>
`

export const show_btn = ()=>{

    const docs_app = document.getElementById("app") as HTMLElement;
    docs_app.innerHTML = btn_tpl;
    const ctr_btns = docs_app.querySelector("#control-btn") as HTMLElement;
    const left_btn = ctr_btns?.querySelector(".left") as HTMLElement;
    left_btn.onclick= ()=>{
        (globalThis as any).rotationOffset -= Math.PI / 180; // 10°
    }

    const right_btn = ctr_btns?.querySelector(".right") as HTMLElement;
    right_btn.onclick= ()=>{
        (globalThis as any).rotationOffset  += Math.PI / 180;
    }
}