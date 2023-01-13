import {defineStore} from 'pinia'
import {request} from "./api/request";
import {EnvInfo} from "./types/model";

const envRef = ref({}as EnvInfo);



export const useEnvStore = defineStore('Env', ()=>{
    onBeforeMount(async () => {
        await getEnv();
    })
    const env = ref(envRef);
    async function getEnv() {
        env.value = await request<EnvInfo>("get_env_info")
    }
    return {env,getEnv}
})

