import { useState, useEffect } from "react";
import "./App.css";

function App() {
  const [isLoading, setIsLoading] = useState(true);

  const goToTranslate = () => {
    try {
      localStorage.setItem('kagiManualRedirect', 'true'); 
    } catch (e) {
    }
    window.location.href = 'https://translate.kagi.com';
  };

  useEffect(() => {
    const loadingTimer = setTimeout(() => {
      setIsLoading(false);
    }, 2500);

    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.key === 'a' && e.ctrlKey && e.shiftKey) {
        goToTranslate();
      }
    };

    window.addEventListener('keydown', handleKeyPress);

    return () => {
      clearTimeout(loadingTimer);
      window.removeEventListener('keydown', handleKeyPress);
    };
  }, []);

  return (
    <main className="container">
      {isLoading && (
        <div className="loading">
          <div className="spinner"></div>
          <h2>Loading Kagi Translate...</h2>
        </div>
      )}

      <iframe
        src="https://translate.kagi.com"
        title="Kagi Translate"
        className={`kagi-webview ${isLoading ? 'hidden' : ''}`}
        onLoad={() => {
          setIsLoading(false);
        }}
      />

      {!isLoading && (
        <button
          onClick={goToTranslate}
          style={{
            position: 'fixed',
            bottom: '20px',
            right: '20px',
            zIndex: 99999,
            backgroundColor: '#007bff',
            color: 'white',
            fontWeight: 'bold',
            padding: '10px 20px',
            borderRadius: '5px',
            fontSize: '14px',
            cursor: 'pointer',
            border: 'none',
            boxShadow: '0 2px 5px rgba(0,0,0,0.2)'
          }}
        >
          Go To Translate
        </button>
      )}
    </main>
  );
}

export default App;
