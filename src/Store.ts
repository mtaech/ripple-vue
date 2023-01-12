import { defineStore } from 'pinia'
import {request} from "./api/request";
import {EnvInfo} from "./types/model";

const envRef = ref({}as EnvInfo);

function getEnv() {
    request<EnvInfo>("get_env_info").then(res=>{
        envRef.value = res
    })
}

export const useEnvStore = defineStore('Env', ()=>{
    const env = ref(envRef);
    function getEnv() {
        request<EnvInfo>("get_env_info").then(res=>{
            envRef.value = res
        })
    }
    return {env,getEnv}
})

