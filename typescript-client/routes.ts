import type {Error,User,UserPayload} from './types';

async function get_user(payload: UserPayload){
	const data = await fetch("/user",{
		headers:{
			'Api-Checksum':'123abc'
		},
		method:'GET',
		body:JSON.stringify(payload)
	})
}