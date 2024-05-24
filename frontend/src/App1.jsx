import "./App.css";
import * as React from "react";
//import axios from "axios";

import { URL, Hosts } from "./Host";
const ALL_HOSTS_API = `${URL}/api/hosts/`;

  // `hosts` STATE DISPATCHER
  /******************************************************************************/
  //function hostsReducer(state, action) {
  //  console.log("state from hostsReducer:", state);
  //  switch (action.type) {
  //    case "HOSTS_FETCH_INIT":
  //      return {
  //        ...state,
  //        isLoading: true,
  //        isError: false,
  //      };
  //    case "HOSTS_FETCH_SUCCESS":
  //      return {
  //        ...state,
  //        isLoading: false,
  //        isError: false,
  //      };
  //    case "HOSTS_FETCH_FAILURE":
  //      return {
  //        ...state,
  //        isLoading: false,
  //        isError: true,
  //      };
  //    case "REMOVE_HOST":
  //    default:
  //      throw new Error();
  //  }
  //}

// MAIN APP
/******************************************************************************/
function App() {
  const [hosts, setHosts] = React.useState( [] );
  const [isLoading, setIsLoading] = React.useState(false);
  const [isError, setIsError] = React.useState(false);

  // INITIATE PAGE, INITIATE `hosts`
  /****************************************************************************/
  React.useEffect(() => {
    setIsLoading(true);

    fetch(`${ALL_HOSTS_API}`)
      .then((response) => response.json())
      .then((result) => {

        console.log("result from useEffect()", result);
        setHosts(result.data.hosts);
        setIsLoading(false);
      })
      .catch(() => setIsError(true));
  }, []);

  return (
    <>
      <h1>Service Monitor</h1>

      {isError && <p>Something went wrong</p>}

      {isLoading ? <p>Loading...</p> : <Hosts hosts={hosts} />}
    </>
  );
}

export default App;
