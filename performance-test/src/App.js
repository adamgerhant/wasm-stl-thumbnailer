import { useEffect, useState, useRef } from 'react'
import {StlViewer} from "react-stl-viewer";
import init, * as wasm from "wasm-stl-thumbnailer"

const App = () => {

  const [file, setFile] = useState()
  const [startTime, setStartTime] = useState(performance.now())

  useEffect(()=>{
    init()
    
  },[])

 
  const handleFileChange = async (event) => {
    const file = event.target.files[0];
    setFile(file)
    const fileUploadTime =performance.now()
    setStartTime(fileUploadTime)
    if (file) {
      try {
        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        //const pngByteStreamStartTime = performance.now();
        const pngByteStream = await wasm.stl_to_png(uint8Array);
        //const pngByteStreamEndTime = performance.now();

        const blob = new Blob([pngByteStream], { type: 'image/png' });
        const dataUrl = URL.createObjectURL(blob);
        const renderTime = (performance.now()-fileUploadTime).toFixed(2)
        console.log(`time for wasm render: ${renderTime}`)

        const img = document.createElement('img');
        img.src =dataUrl;
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
        const renderTime = (performance.now()-startTime).toFixed(2)

        console.log(`time for stl viewer render ${renderTime}`)

        const imgElement = document.createElement('img');
        imgElement.src = image;
        const label = document.createElement('p');
        label.textContent = `Render time: ${renderTime} ms`

        const div = document.getElementById('react-stl-renders');
        div.appendChild(imgElement);
        div.appendChild(label)

      }
     

    } , 10)
  }
  
  const fileURL = file?window.URL.createObjectURL(file):"";



  return(
    <div style={{ minHeight:"100vh"}}>
      <div style={{padding:"5px", display:"flex", height:"500px", alignItems:"center", justifyContent:"center"}}>     
          <input type="file" onChange={handleFileChange}/>
          <div style={{ border:"1px solid black", width:"512px", height:"512px"}}>
            {fileURL&&
            
              <StlViewer
                url={fileURL}
                style={{
                  width:"512px",
                  height:"512px"
                }}
                
                onFinishLoading={()=>captureImage()}
                canvasId='3d_canvas'
              />
            
            }
          </div>
      </div>
      <div style={{display:"flex"}}>
        <div style={{display:"flex", flexDirection:"column", alignItems:"center", width:"50%"}} id="web-assembly-renders">
            <h3 style={{fontFamily:"sans-serif"}}>Web Assembly Renders</h3>
        </div>
        <div style={{display:"flex", flexDirection:"column", alignItems:"center", width:"50%"}} id="react-stl-renders">
            <h3 style={{fontFamily:"sans-serif"}}>React STL Viewer Renders</h3>
        </div>
      </div>
      
    </div>
   
    
  )
}


export default App
