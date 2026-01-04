export default function SideBar() {
    return (
        <div style={style}>Side bar</div>
    )
}

const style: React.CSSProperties = {
    border: "5px solid red",
    borderRadius: "200px",
    minWidth: "15%",
    textAlign: "center",
    display: "flex",
    justifyContent: 'center',
    alignItems: "center"
}