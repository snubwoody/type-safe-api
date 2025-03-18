export interface UserPayload{
	id: string,
}

export interface Error{
	description: string,
	details: string,
	code: number,
}

export interface User{
	name: string,
	created_at: string,
	email: string,
	id: number,
}

