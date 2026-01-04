export default function PageContent() {

    const elements = Array.from({ length: 20 }, (_, i) => `Content ${i + 1}`);

    return (
        <div style={containerStyle}>
            <h2 style={titleStyle}>✨ Page Content ✨</h2>

            <div style={listStyle}>
                {elements.map((item, index) => (
                    <div
                        key={index}
                        style={elementStyle}
                    >
                        {item}
                    </div>
                ))}
            </div>
        </div>
    );
}


const containerStyle: React.CSSProperties = {
    borderRadius: "24px",
    padding: "1.5rem",
    minWidth: "80%",
    background: "linear-gradient(135deg, #0f172a, #1e293b)",
    boxShadow: "0 10px 30px rgba(0,0,0,0.3)",
    textAlign: "center",
};

const titleStyle: React.CSSProperties = {
    color: "#e0f2fe",
    marginBottom: "1rem",
    letterSpacing: "1px",
};

const listStyle: React.CSSProperties = {
    display: "flex",
    justifyContent: "center",
    gap: "1rem",
    flexWrap: "wrap",
    alignItems: "center"
};

const elementStyle: React.CSSProperties = {
    padding: "0.75rem 1.5rem",
    borderRadius: "100px",
    background: "linear-gradient(150deg, #f59e0b, #f97316)",
    color: "#1e293b",
    fontWeight: 600,
    cursor: "pointer",
    // transition: "all 0.25s ease",
    // boxShadow: "0 4px 10px rgba(0,0,0,0.2)",
};
