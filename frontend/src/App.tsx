import { useState } from "react";
import ScientistGallery from "./pages/AmazingScientists";
import Login from "./pages/Login";

function App() {

  const [isLoggedIn, setIsLoggedIn] = useState<boolean>(false);
  return (
    <>
    { isLoggedIn ? <ScientistGallery/> : <Login setLogin={setIsLoggedIn}/>}
    </>
  )
}

export default App;
