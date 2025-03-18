export interface Error{
	description: string,
	details: string,
	code: number,
}

export interface OrderPayload{
	email: string,
	cart_id: string,
	name: string,
}

export interface User{
	id: number,
	created_at: string,
	name: string,
	email: string,
}

export interface UserPayload{
	id: string,
}

export class Client {
	checksum: string
	constructor(checksum: string,) {		this . checksum = checksum ;
	}
	async create_user(payload: UserPayload,): Promise<User> {
		try { const response = await fetch ("https://example.com/user" , { headers : { "Api-Checksum" : this . checksum } , method : "POST" , body : JSON . stringify (payload) }) ; if (response . ok) { const body : User = await response . json () ; return body ; } else { const error = await response . json () ; throw error ; } } catch (err) { throw err ; }
	}
	async submit_order(payload: OrderPayload,): Promise<string> {
		try { const response = await fetch ("https://example.com/order" , { headers : { "Api-Checksum" : this . checksum } , method : "POST" , body : JSON . stringify (payload) }) ; if (response . ok) { const body : string = await response . json () ; return body ; } else { const error = await response . json () ; throw error ; } } catch (err) { throw err ; }
	}
}