import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { Dashboard } from '../../features/dashboard';

export function AppRoutes() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Dashboard />} />
      </Routes>
    </BrowserRouter>
  );
}
