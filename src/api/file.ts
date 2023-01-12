import {nanoid} from "nanoid";
import {Attachment} from "../types/model";
import {request} from "./request";
import {BaseDirectory, writeBinaryFile} from "@tauri-apps/api/fs";
const BASE_ATT_DIR = "attachment/"
export function save_att(file:File) {
    let name = file.name;
    let size = file.size;
    let type = file.type;
    let index = name.lastIndexOf(".");
    let info = {
        id:nanoid(10),
        file_name:name,
        file_size:size,
        content_type:type
    } as Attachment;
    if (index>-1){
        info.suffix = name.slice(index+1);
    }
    file.arrayBuffer().then(buf=>{
        let path = BASE_ATT_DIR+info.id+"."+info.suffix;
        writeBinaryFile(path,buf,{dir:BaseDirectory.Picture}).then(msg=>{
            console.log("base di",BaseDirectory.Picture.toString())
            info.file_path = path
            request("add_file",{att:info}).then(res=>{
                console.log("res",res)
            })
        })
    })
}

