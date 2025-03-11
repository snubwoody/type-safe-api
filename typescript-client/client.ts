
async function main(){
	const response = await fetch("http://localhost:1000");
	console.log(response.status);
}

main()