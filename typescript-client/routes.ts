import type {Error,User,UserPayload} from './types';


export class Client{
	checksum: string
	constructor(checksum:string){}
	
	async create_user(payload: UserPayload): Promise<User | Error>{
		const data = await fetch("/user",{
			headers:{
				'Api-Checksum':this.checksum
			},
			method:'POST',
			body:JSON.stringify(payload)
		})
	
		return await data.json()
	}
}

let client = new Client('')