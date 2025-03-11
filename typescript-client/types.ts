export interface User{
	id: number,
	created_at: string,
	email: string,
	name: string,
}

export interface UserPayload{
	id: string,
}

export interface Error{
	details: string,
	code: number,
	description: string,
}

