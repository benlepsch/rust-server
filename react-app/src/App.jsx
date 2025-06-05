import { React, useState } from 'react'
import { Link, BrowserRouter, Routes, Route } from 'react-router-dom';

function Home() {
    return <p>Welcome home</p>;
}

function PageNotFound() {
    return <p>Not a real page bozo</p>;
}

function Main() {
    return (
        <Routes>
            <Route exact path='/' element={<Home />}></Route>

            <Route path='*' element={<PageNotFound />}></Route>
        </Routes>
    );
}


function App() {
    return (
        <BrowserRouter>
            <Main />
        </BrowserRouter>
    );
}

export default App;
