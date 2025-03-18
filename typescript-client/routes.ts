import {Client, UserPayload} from './types';



let client = new Client('')

let payload:UserPayload = {
	id: '',
	
}

try{
	const result = await client.create_user(payload)
} catch(e){

}
