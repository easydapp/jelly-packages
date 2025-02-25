import { useRoutes } from 'react-router-dom';
import './App.scss';
import routes from './routes';

function App() {
    const views = useRoutes(routes);

    return <>{views}</>;
}

export default App;
