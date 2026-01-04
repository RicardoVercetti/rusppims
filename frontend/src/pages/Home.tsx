import PageContent from "./PageContent";
import SideBar from "./SideBar";

export default function Home() {
    return (
        <div style={homeStyle} className="main-page">
            <SideBar/>
            <PageContent/>
        </div>        
    );
}

const homeStyle: React.CSSProperties = {
    display: "flex",
    minHeight: "80vh",
    minWidth: "90vw"
}