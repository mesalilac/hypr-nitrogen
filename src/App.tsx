import Header from '@components/Header';
import Settings from '@components/Settings';
import ThumbnailsList from '@components/ThumbnailsList';
import { Toaster } from 'solid-toast';
import '@styles/App.css';
import { GlobalContextProvider } from './store';

function App() {
    return (
        <main class='container'>
            <GlobalContextProvider>
                <Toaster position='bottom-right' />
                <Header />
                <ThumbnailsList />
                <Settings />
            </GlobalContextProvider>
        </main>
    );
}

export default App;
