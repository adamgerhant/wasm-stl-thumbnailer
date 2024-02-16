import { useEffect, useState, useRef } from 'react'
import {StlViewer} from "react-stl-viewer";
import init, * as wasm from "wasm-stl-thumbnailer"
import Magnet_holder from "./Magnet_holder.stl"
const Performance = () => {
  
  const [file, setFile] = useState()
  const [startTimeState, setStartTimeState] = useState()

  useEffect(()=>{
    init()
  },[])

 
  const handleFileChange = async (event) => {
    const file = event.target.files[0];
    setFile(file)
    const startTime = performance.now()
    setStartTimeState(startTime)
    wasmThumbnailer(file, startTime) 
  }
  
  const wasmThumbnailer = async (file, startTime)=>{
    if (file) {
      try {
        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        const pngByteStream = await wasm.stl_to_png(uint8Array);

        const blob = new Blob([pngByteStream], { type: 'image/png' });
        const dataUrl = URL.createObjectURL(blob);
        const renderTime = (performance.now()-startTime).toFixed(2)
        console.log(`time for wasm render: ${renderTime}`)

        const img = document.createElement('img');
        img.src =dataUrl;
        img.style.border = '1px solid black'; 

        const label = document.createElement('p');
        label.textContent = `Render time: ${renderTime} ms`

        const div = document.getElementById('web-assembly-renders');
        div.appendChild(img);
        div.appendChild(label)
      

      } catch (error) {
        console.error('Error reading file:', error.message);
      }
    }
  } 

  const captureImage = () => {

    //set 10ms delay since model will not be visibile if called directly. on low power devices the delay has to be increased.
    setTimeout(() => {
      var canvas = document.getElementById('3d_canvas')?.firstChild?.firstChild;
      if(canvas){
        const image = canvas.toDataURL();
        const renderTime = (performance.now()-startTimeState).toFixed(2)

        console.log(`time for stl viewer render ${renderTime}`)

        const img = document.createElement('img');
        img.src = image;
        img.style.border = '1px solid black'; 
        const label = document.createElement('p');
        label.textContent = `Render time: ${renderTime} ms`

        const div = document.getElementById('react-stl-renders');
        div.appendChild(img);
        div.appendChild(label)

      }
     

    } , 10)
  }
  
  const loadFile = () =>{
    fetch(Magnet_holder).then(res => {
      return res.blob();
    }).then(blob => {
      const file = new File([blob], "Magnet_holder", { type: "application/octet-stream" });
      setFile(file)
      const startTime = performance.now()
      setStartTimeState(startTime)
      wasmThumbnailer(file, startTime)
    })
    .catch(error => {
      console.error('Error fetching the file:', error);
    });
    
  }
  const fileURL = file?window.URL.createObjectURL(file):"";



  return(
    <div style={{display:"flex"}}>
      <div style={{padding:"5px", display:"flex", flexDirection:"column", alignItems:"center"}}>     
          <div style={{display:"flex", width:"500px", padding:"50px"}}>
            <input type="file" onChange={handleFileChange}/>
            <button onClick={()=>loadFile()} style={{width:"200px"}}>Load Example File</button>
          </div>
          <h3 style={{fontFamily:"sans-serif"}}>react-stl-viewer component</h3>

          <div style={{ border:"1px solid black", width:"512px", height:"512px"}}>
            {fileURL&&
            
              <StlViewer
                url={fileURL}
                style={{
                  width:"512px",
                  height:"512px"
                }}
                orbitControls
                onFinishLoading={()=>captureImage()}
                canvasId='3d_canvas'
              />
            
            }
          </div>
      </div>
      <div style={{display:"flex", flex:"1"}}>
        <div style={{display:"flex", flexDirection:"column", alignItems:"center", width:"50%"}} >
            <h3 style={{fontFamily:"sans-serif"}}>Web Assembly Renders</h3>
            <div style={{display:"flex", flexDirection:"column-reverse", alignItems:"center"}} id="web-assembly-renders"/>
        </div>
        <div style={{display:"flex", flexDirection:"column", alignItems:"center", width:"50%"}}>
            <h3 style={{fontFamily:"sans-serif"}}>react-stl-viewer Renders</h3> 
            <div style={{display:"flex", flexDirection:"column-reverse", alignItems:"center"}} id="react-stl-renders"/>
        </div>
      </div>
      
    </div>
   
    
  )
}


export default Performance
