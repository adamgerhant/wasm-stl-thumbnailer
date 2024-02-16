## About
Generates a PNG render from an STL file. 2-3x faster than Three.js for first render due to GPU pre-initilization, 10-20% faster for future renders. Built with Web Assembly and WebGPU. 

## Usage
```
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
```

## Performance
Performance was tested with the following: https://adamgerhant.github.io/wasm-stl-thumbnailer/
This page compares time to render after reloading the page. The wasm-stl-thumbnailer is compared to the [react-stl-thumbnailer](https://www.npmjs.com/package/react-stl-viewer)
Various file sizes were tested, and the time is an average of 5 first renders. The full data is available on [this Google Sheet](https://docs.google.com/spreadsheets/d/1pVuQIuG0zfEBlZs5bSOI_l7UgPihVZnMvkIie5UpIWo/edit?usp=sharing) 

![optimized bar chart](https://github.com/adamgerhant/wasm-stl-thumbnailer/assets/116332429/ddeaa5c2-73ab-408d-b334-58a578dd50d0)
![optimized percent decrease](https://github.com/adamgerhant/wasm-stl-thumbnailer/assets/116332429/dff6bb8c-b178-4744-bc17-91c0ed8c66bb)
