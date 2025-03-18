export interface UserPayload{
	id: string,
}

export interface Error{
	details: string,
	code: number,
	description: string,
}

export interface User{
	id: number,
	name: string,
	email: string,
	created_at: string,
}

export class Client {
	checksum: string
	constructor(checksum: string,) {		this . checksum = checksum ;
	}
	async create_user(payload: UserPayload,): Promise<User> {
		try { const response = await fetch ("https://facebook.com/user" , { headers : { "Api-Checksum" : this . checksum } , method : "POST" , body : JSON . stringify (payload) }) ; if (response . ok) { const body : User = await response . json () ; return body ; } else { const error = await response . json () ; throw error ; } } catch (err) { throw err ; }
	}
}