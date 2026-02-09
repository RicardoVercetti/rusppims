import "./css/sidebar.css";

export default function SideBar() {
  return (
    <aside className="sidebar">
      <h2 className="logo">PPIMS</h2>

      <nav className="nav">
        <button className="nav-item active">Dashboard</button>
        <button className="nav-item">Customers</button>
        <button className="nav-item">Transactions</button>
        <button className="nav-item">Reports</button>
        <button className="nav-item">Settings</button>
      </nav>

      <div className="sidebar-footer">
        <span>v1.0.0</span>
      </div>
    </aside>
  );
}
