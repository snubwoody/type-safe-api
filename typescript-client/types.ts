export interface UserPayload{
	id: string,
}

export interface User{
	name: string,
	email: string,
	id: number,
	created_at: string,
}

export interface Error{
	code: number,
	description: string,
	details: string,
}

