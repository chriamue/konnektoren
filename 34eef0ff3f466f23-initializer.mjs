let b=`loading-message`,c=`Loading...`;function a(){let a=0;return {onStart:()=>{a=performance.now();document.getElementById(`loading-container`).style.display=`block`;document.getElementById(b).textContent=c},onProgress:({current:a,total:d})=>{const e=document.getElementById(`loading-progress`);const f=document.getElementById(b);if(d){const b=Math.round((a/d)*100);e.style.width=`${b}%`;f.textContent=`Loading... ${b}%`}else{f.textContent=c}},onFailure:a=>{console.warn(`Loading... failed!`,a)}}}export{a as default}