import { Excalidraw, MainMenu } from "@excalidraw/excalidraw";
import { OrderedExcalidrawElement } from "@excalidraw/excalidraw/element/types";
import "@excalidraw/excalidraw/index.css";
import { AppState, BinaryFiles, ExcalidrawImperativeAPI } from "@excalidraw/excalidraw/types";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import { IoClose } from "react-icons/io5";
import { TbCheck, TbPencil } from "react-icons/tb";

function App() {
	const [readOnly, setReadOnly] = useState(false);

	let timeout = useRef<number | undefined>(undefined);
	const api = useRef<ExcalidrawImperativeAPI | null>(null);

	const handleChange = async (elements: readonly OrderedExcalidrawElement[], appState: AppState, files: BinaryFiles) => {
		clearTimeout(timeout.current);
		timeout.current = setTimeout(async () => {
			(appState as any)["readonly"] = api.current!.getAppState().viewModeEnabled;
			await invoke("save_state", { state: { elements: elements, state: appState, files } });
		}, 500);
	};

	const getState = async () => {
		const state = await invoke<{
			elements: readonly OrderedExcalidrawElement[];
			state: AppState;
			files: BinaryFiles;
			readonly: boolean;
		} | null>("get_state");

		if (state) {
			const { state: appState, ...rest } = state;
			setReadOnly((appState as any).readonly || false);
			const { collaborators, readonly, ...restState } = appState as any;

			return { ...rest, appState: restState };
		}

		return null;
	};

	useEffect(() => {
		getState().then((state) => {
			if (state) {
				const { files, ...rest } = state;
				api.current!.updateScene(rest);
				api.current!.addFiles(Object.values(files));
			}
		});
	}, []);

	const closeNote = async () => {
		await invoke("close");
	};

	useEffect(() => {
		if (!api.current) return;
		api.current.updateScene({ appState: { viewModeEnabled: readOnly } });
		handleChange([], api.current.getAppState(), {});
	}, [readOnly]);

	return (
		<div className="content">
			<div className="title" data-tauri-drag-region />
			<Excalidraw
				onChange={(elements, appState, files) => {
					handleChange(elements, appState, files);
				}}
				excalidrawAPI={(a) => {
					api.current = a;
				}}
			>
				<MainMenu>
					<MainMenu.Group title="Files">
						<MainMenu.DefaultItems.LoadScene />
						<MainMenu.DefaultItems.Export />
						<MainMenu.DefaultItems.SaveToActiveFile />
						<MainMenu.DefaultItems.SaveAsImage />
					</MainMenu.Group>
					<MainMenu.Group title="Canvas">
						<MainMenu.DefaultItems.ClearCanvas />
						<MainMenu.DefaultItems.ToggleTheme />
						<MainMenu.DefaultItems.ChangeCanvasBackground />
					</MainMenu.Group>
					<MainMenu.Group title="Settings">
						<MainMenu.Item onClick={() => setReadOnly(!readOnly)}>
							{readOnly ? <TbPencil /> : <TbCheck />}
						</MainMenu.Item>
						<MainMenu.Item onClick={closeNote}>
							<IoClose />
						</MainMenu.Item>
					</MainMenu.Group>
				</MainMenu>
			</Excalidraw>
		</div>
	);
}

export default App;
