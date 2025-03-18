export interface UserPayload{
	id: string,
}

export interface User{
	name: string,
	id: number,
	created_at: string,
	email: string,
}

export interface Error{
	description: string,
	code: number,
	details: string,
}

class Client {
	async create_user(body: UserPayload,): Promise<User> {
		try { const response = await fetch ("https://facebook.com/user") ; if (response . ok) { const user : = await response . json () ; return user ; } else { const error = response . json () ; throw error ; } } catch (err) { throw err ; }
	}
}