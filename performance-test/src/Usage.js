import { useEffect, useState} from 'react'
import init, * as wasm from "wasm-stl-thumbnailer"

const Usage = () => {

    //initialze Web Assembly
    useEffect(()=>{
        init()
    },[])

    const handleFileChange = async (event) => {
        const file = event.target.files[0];
        if (file) {
            try {
                //convert file to byte stream
                const arrayBuffer = await file.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);

                //get png byte stream from stl byte stream
                const pngByteStream = await wasm.stl_to_png(uint8Array);

                //add image to document
                const blob = new Blob([pngByteStream], { type: 'image/png' });
                const dataUrl = URL.createObjectURL(blob);
                const img = document.createElement('img');
                img.src =dataUrl;
                document.body.appendChild(img);
            
            } catch (error) {
                console.error('Error reading file:', error.message);
            }
        }
    }
  
    return(
        <input type="file" onChange={handleFileChange}/>      
    )
}

export default Usage
