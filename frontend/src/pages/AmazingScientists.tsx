function Profile() {
    return (
        <img
      src="https://i.imgur.com/MK3eW3As.jpg"
      alt="Katherine Johnson"
        />
    );
}

export default function ScientistGallery() {
    return (
        <>
        <section>
            <h1> Some scientist names</h1>
            <Profile/>
            <Profile/>
            <Profile/>
        </section>
        </>
    );
} 