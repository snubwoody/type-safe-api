export interface Error{
	details: string,
	description: string,
	code: number,
}

export interface UserPayload{
	id: string,
}

export interface User{
	id: number,
	created_at: string,
	email: string,
	name: string,
}

class Client {
	async create_user(body: UserPayload,): Promise<User> {
		try { const response = await fetch ("https://facebook.com/user") ; if (response . ok) { const user : User = await response . json () ; return user ; } else { const error = response . json () ; throw error ; } } catch (err) { throw err ; }
	}
}