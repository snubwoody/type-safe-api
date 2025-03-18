export interface Error{
	details: string,
	description: string,
	code: number,
}

export interface UserPayload{
	id: string,
}

export interface User{
	created_at: string,
	email: string,
	name: string,
	id: number,
}

