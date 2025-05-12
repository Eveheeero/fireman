import { useEffect } from "react";
import { open } from '@tauri-apps/plugin-dialog';

function Nav() {
  function closeAllDropdowns() {
    document.querySelectorAll('.dropdown-menu').forEach(menu => {
      menu.classList.add("hidden");
    });
  }

  function openDropdown(e: React.MouseEvent<HTMLButtonElement>) {
    const menu = e.currentTarget.nextElementSibling;
    if (menu?.classList.contains('dropdown-menu')) {
      const isAlreadyOpen = !menu.classList.contains('hidden');
      closeAllDropdowns();
      if (!isAlreadyOpen) {
        menu.classList.remove('hidden');
      }
    }
  }

  function registerEventListener() {
    const handleDocumentClick = function (event: MouseEvent) {
      const target = event.target;
      if (target == null) { return; }
      let targetCasted = target as HTMLElement;
      if (!targetCasted.closest('.dropdown-group')) {
        closeAllDropdowns();
      }
    };
    document.addEventListener('click', handleDocumentClick);

    return () => {
      document.removeEventListener('click', handleDocumentClick);
    };
  }

  async function click() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    console.log(file);
  }

  useEffect(() => {
    const cleanup = registerEventListener();
    return cleanup; // cleanup
  }, []); // 빈 배열: 마운트 시 한 번 실행, 언마운트 시 클린업 실행

  return (<nav className="navbar shadow-md mx-1 flex justify-start">
    <div className="relative dropdown-group">
      <button onClick={(e) => openDropdown(e)} className="dft-btn mx-0.5">Files</button>
      <div className="dropdown dropdown-menu hidden">
        <button onClick={(_e) => click()} className="dropdown-item">Open</button>
      </div>
    </div>
    <div className="relative dropdown-group">
      <button onClick={(e) => openDropdown(e)} className="dft-btn mx-0.5">Test</button>
      <div className="dropdown dropdown-menu hidden">
        <div className="dropdown-item">Testitem</div>
      </div>
    </div>
  </nav>)
}

export default Nav;