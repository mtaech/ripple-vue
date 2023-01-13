import {invoke} from "@tauri-apps/api";
import {InvokeArgs} from "@tauri-apps/api/tauri";
import {ApiResult} from "../types/model";
import {RouteParams} from "vue-router";
import {Notification} from '@arco-design/web-vue';


export function getRouteStingParam(params:RouteParams,key:string):string {
    console.log("params",params)
    let param = params[key];
    if (param === undefined || param === null){
        return "";
    }
    if (typeof param === 'string'){
        return param;
    }else {
        return param.join(",");
    }
}

export async function request<T>(command: string, args?: InvokeArgs): Promise<T> {

    try {
        console.debug("args",args)
        let apiResult = await invoke<ApiResult<T>>(command, args);
        if (apiResult.code == 200) {
            // @ts-ignore
            return apiResult.data;
        } else {
            Notification.error({
                title: '操作失败',
                content: apiResult.msg,
                duration: 2500
            })
            return Promise.reject(apiResult.msg);
        }
    } catch (e) {
        Notification.error({
            title: '操作失败',
            content: "操作失败请重试",
            duration: 2500
        })
        console.error("error", e)
        return Promise.reject(e);
    }

}