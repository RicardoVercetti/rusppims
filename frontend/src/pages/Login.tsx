import { useState } from "react";

interface LoginProps {
    setLogin: (val: boolean) => void
}

async function login(body: {username: string, password: string}) {
  const url = "http://localhost:3000/login";
  const res =  await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });
  if (!res.ok) {
    console.log("response is not OK");
    throw Error("Login failed");
  }

  console.log("response is OK");
  const data = await res.json();
  return data;
}

export default function Login({setLogin} : LoginProps) {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();     
        console.log(`username: ${username}`);
        console.log(`password: ${password}`);
        
        try {
            const data = await login({username, password});
            console.log("data: ", data);
            setLogin(true);
            console.log("set login already")

        } catch (err) {
            console.log(err);
        }
    }

    return (
        <div className="container">
        <div className="form">
            <form onSubmit={handleSubmit}>
            <h3>Sign in to PPIMS</h3>
            <input id="name" type="text" value={username} placeholder="Username" onChange={e => setUsername(e.target.value)} required/>
            <input id="pass" type="password" value={password} placeholder="Password" onChange={e => setPassword(e.target.value)} required/>
            <button type="submit">Submit</button>
            </form>
        </div>
        </div>
    )
}