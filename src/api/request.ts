import {invoke} from "@tauri-apps/api";
import {InvokeArgs} from "@tauri-apps/api/tauri";
import {ApiResult} from "../types/model";
import {ConfigProviderProps, createDiscreteApi, darkTheme, lightTheme} from 'naive-ui'
import {RouteParams} from "vue-router";


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
    const themeRef = ref<'light' | 'dark'>('light')
    const configProviderPropsRef = computed<ConfigProviderProps>(() => ({
        theme: themeRef.value === 'light' ? lightTheme : darkTheme
    }))
    const {notification} = createDiscreteApi(
        ['notification'],
        {
            configProviderProps: configProviderPropsRef
        }
    )
    try {
        console.debug("args",args)
        let apiResult = await invoke<ApiResult<T>>(command, args);
        if (apiResult.code == 200) {
            // @ts-ignore
            return apiResult.data;
        } else {
            notification.error({
                content: '操作失败',
                meta: apiResult.msg,
                duration: 2500,
                keepAliveOnHover: true
            })
            return Promise.reject(apiResult.msg);
        }
    } catch (e) {
        notification.error({
            content: '操作失败',
            meta: '操作失败请重试',
            duration: 2500,
            keepAliveOnHover: true
        })
        console.error("error", e)
        return Promise.reject(e);
    }

}