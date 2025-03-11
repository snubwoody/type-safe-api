export interface User{
	created_at: string,
	id: number,
	name: string,
	email: string,
}

export interface UserPayload{
	id: string,
}

export interface Error{
	details: string,
	description: string,
	code: number,
}

