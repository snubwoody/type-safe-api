export interface User{
	created_at: string,
	email: string,
	id: number,
	name: string,
}

export interface Error{
	code: number,
	description: string,
	details: string,
}

export interface UserPayload{
	id: string,
}

