import { useState } from "react";
import Login from "./pages/Login";
import Home from "./pages/Home";
import './app.css';

function App() {

  const [isLoggedIn, setIsLoggedIn] = useState<boolean>(false);
  return (
    <>
    { isLoggedIn ? <Home/> : <Login setLogin={setIsLoggedIn}/>}
    </>
  )
}

export default App;
