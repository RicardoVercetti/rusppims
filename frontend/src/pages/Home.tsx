import SideBar from "./SideBar";
import PageContent from "./PageContent";
import "./css/home.css";

export default function Home() {
  return (
    <div className="home-container">
      <SideBar />
      <PageContent />
    </div>
  );
}
