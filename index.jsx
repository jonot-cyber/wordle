import React, { useState, useRef, useEffect } from "react";
import init, { get_best_word, guess_format, initThreadPool } from "./pkg/wordle.js";
import { createRoot } from "react-dom/client";

const root = createRoot(document.getElementById("app"));

const baseWord = [
  { "info": 2, "letter": "?" },
  { "info": 2, "letter": "?" },
  { "info": 2, "letter": "?" },
  { "info": 2, "letter": "?" },
  { "info": 2, "letter": "?" },
];

function LetterBox(props) {
  const ref = useRef(null);

  useEffect(() => {
    if (props.focus) {
      ref.current.focus();
    } else {
      ref.current.blur();
    }
  }, [props.focus])

  return <div onFocus={props.onF}
              tabIndex={0}
              ref={ref}
              onKeyDown={props.onLetterChange}
              onClick={props.onInfoChange}
              className={"focus:border-black w-24 p-0 h-24 flex justify-center items-center text-white font-bold text-5xl rounded-lg " + (props.info == 0 ? "bg-green-500" : props.info == 1 ? "bg-yellow-500" : "bg-gray-500")}>
    <span className="select-none">{props.letter}</span>
  </div>
}

function WordBox(props) {
  return <div className="p-4 h-fit w-fit flex gap-4 rounded-lg bg-gray-200">
    {props.data.map((d, i) => {
      return <LetterBox onF={() => props.onF(i)}
                        focus={props.focus && i == props.focusI}
                        onInfoChange={() => props.onInfoChange(i)}
                        onLetterChange={e => props.onLetterChange(i, e)}
                        key={i}
                        info={d.info}
                        letter={d.letter} />
    })}
  </div>
}

function WordsBox(props) {
  const [focusI, setFocusI] = useState(0);
  const [focusJ, setFocusJ] = useState(0);
  const addButtonRef = useRef(null);

  const [data, setData] = useState([baseWord]);
  const [suggestedGuess, setSuggestedGuess] = useState("");

  function setInfo(i, j, up) {
    setData(data.map((v, i2) => {
      if (i2 != i) {
        return v;
      }
      return v.map((v2, j2) => {
        if (j2 != j) {
          return v2;
        }
        return { ...v2, "info": (v2.info + (up ? 1 : -1)) % 3 };
      })
    }))
  }

  function changeInfo(i, j) {
    setInfo(i, j, true);
  }

  function changeLetter(i, j, e) {
    if (e.key == "ArrowUp") {
      setInfo(i, j, true);
      e.preventDefault();
      return;
    } else if (e.key == "ArrowDown") {
      setInfo(i, j, false);
      e.preventDefault();
      return;
    }
    if (e.key.length != 1 || e.key == " ") {
      return;
    }

    setData(data.map((v, i2) => {
      if (i2 != i) {
        return v;
      }
      return v.map((v2, j2) => {
        if (j2 != j) {
          return v2;
        }
        return { ...v2, "letter": e.key.toUpperCase() };
      });
    }))
    if (focusJ == 4) {
      addButtonRef.current.focus();
    } else {
      setFocusJ(focusJ + 1);
    }
  }

  function addWordBox() {
    setData([...data, baseWord])
    setFocusI(focusI + 1);
    setFocusJ(0);
  }

  function guess() {
    const words = data.map(i => i.map(j => j["letter"])
                                 .join("")
                                 .toLowerCase())
                      .filter(i => !i.includes("?"));
    const infos = data.map(i => i.map(j => j["info"]))
                      .flat();
                      
    const word = get_best_word(words, infos);
    setSuggestedGuess(word);
  }

  function focusHandler(i, j) {
    setFocusI(i);
    setFocusJ(j);
  }

  return <div>
    <div className="flex flex-col gap-2 m-4">
      {data.map((v, i) => {
        return <WordBox onF={j => focusHandler(i, j)}
                        focusI={focusJ}
                        focus={i == focusI}
                        onInfoChange={j => changeInfo(i, j)}
                        onLetterChange={(j, e) => changeLetter(i, j, e)}
                        key={i}
                        data={v} />
      })}
    </div>
    <div className="flex m-4 gap-4">
      <button className="select-none bg-gray-200 text-5xl rounded-md"
              onClick={addWordBox}
              ref={addButtonRef}>+</button>
      <button onClick={guess}
              className="select-none bg-green-400 text-5xl rounded-md grow">Guess</button>
    </div>
    <p><span className="select-none">Guess: </span>{suggestedGuess}</p>
  </div>
}

async function start() {
  root.render(<div><WordsBox /></div>);
  await init("./wordle_bg.wasm");
  await initThreadPool(navigator.hardwareConcurrency);
}
start().then();