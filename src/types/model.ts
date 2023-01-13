export interface ApiResult<T> {
    code: number,
    msg: string,
    data: T | null
}

interface BaseModel {
    id: string,
    create_time: string,
    modified_time: string
}

export interface BookModel extends BaseModel {
    name: string,
    description: string,
    cover_id:string
    cover_path:string,
    text_count:number
}

export interface Page<T> {
    datas: T[],
    page_no: number,
    page_size: number,
    total: number,
    pages: number,
}


export interface PageReq {
    page_no: number,
    page_size: number
}


export interface Item {
    type?: string,
    title?: string,
    icon?: string,
    action?: () => void,
    isActive?: () => boolean
}

export interface Chapter extends BaseModel {
    name: string,
    text_content: string,
    html_content: string,
    volume_id: string,
    volume_name: string,
    book_id: string,
    book_name: string,
    text_count: number
}

export interface EditorInfo {
    text: string,
    html: string,
    text_count: number,
    word_count: number
}

export interface Attachment extends BaseModel {
    file_name: string,
    content_type: string,
    file_size: number,
    suffix: string
    file_path: string
}

export interface EnvInfo {
    home_dir_path: string,
    config_dir_path: string,
    log_dir_path: string,
    db_dir_path: string,
    att_dir_path:string,
    cover_dir_path:string,
}