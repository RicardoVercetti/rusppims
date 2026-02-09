import { useEffect, useState } from "react";
import "./css/pageContent.css";

/* ---------- Types ---------- */

interface DashboardData {
  transactions_today: number;
  total_customers: number;
  kyc_y1: number;
  kyc_y2: number;
  kyc_y3: number;
}

interface DashboardResponse {
  status: string;
  message: string;
  data: DashboardData;
}

/* ---------- Component ---------- */

export default function PageContent() {
  const [stats, setStats] = useState<DashboardData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchDashboard = async () => {
      try {
        const res = await fetch("http://localhost:3000/dashboard");

        if (!res.ok) {
          throw new Error("Failed to fetch dashboard data");
        }

        const json: DashboardResponse = await res.json();

        if (json.status !== "00") {
          throw new Error(json.message);
        }

        setStats(json.data);
      } catch (err: unknown) {
        console.error(err);
        setError(err instanceof Error ? err.message : "Unknown error");
      } finally {
        setLoading(false);
      }
    };

    fetchDashboard();
  }, []);

  /* ---------- UI States ---------- */

  if (loading) {
    return (
      <main className="content">
        <p>Loading dashboard...</p>
      </main>
    );
  }

  if (error || !stats) {
    return (
      <main className="content">
        <p className="error-text">Error: {error ?? "Unknown error"}</p>
      </main>
    );
  }

  /* ---------- Render ---------- */

  return (
    <main className="content">
      <header className="content-header">
        <h1>Dashboard</h1>
        <p>Welcome back 👋</p>
      </header>

      <section className="stats">
        {/* Transactions Today */}
        <div className="stat-card">
          <span className="stat-title">Transactions Today</span>
          <strong className="stat-value">
            {stats.transactions_today}
          </strong>
        </div>

        {/* Total Customers */}
        <div className="stat-card">
          <span className="stat-title">Total Customers</span>
          <strong className="stat-value">
            {stats.total_customers}
          </strong>
        </div>

        {/* KYC Status */}
        <div className="stat-card">
          <span className="stat-title">KYC Status</span>

          <div className="kyc-breakdown">
            <div>Y1 <strong>{stats.kyc_y1}</strong></div>
            <div>Y2 <strong>{stats.kyc_y2}</strong></div>
            <div>Y3 <strong>{stats.kyc_y3}</strong></div>
          </div>
        </div>
      </section>
    </main>
  );
}
