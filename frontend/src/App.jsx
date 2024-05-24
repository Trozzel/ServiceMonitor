import "./App.css";
import * as React from "react";

// LOCAL IMPORTS
import URL from "./components/helper";
import SVHeader from "./components/Header";
import Hosts from "./components/Host/Host";
const API_ALL_STATUSES = `${URL}/api/svc/get_latest_statuses`;

// `svcStatuses` REDUCER
/******************************************************************************/
function svcStatusesReducer(state, action) {
  switch (action.type) {
    case "STATUSES_FETCH_INIT":
      return {
        ...state,
        triggerLoad: true,
        isLoading: true,
        isError: false,
        error: "",
      };
    case "STATUSES_FETCH_SUCCESS":
      console.log("data inside of reducer:", action.payload);
      return {
        ...state,
        triggerLoad: false,
        isLoading: false,
        isError: false,
        data: action.payload,
        error: "",
      };
    case "STATUSES_FETCH_FAILURE":
      return {
        ...state,
        triggerLoad: false,
        isLoading: false,
        isError: true,
        error: action.payload,
      };
    case "STATUSES_REMOVE_HOST":
      return {
        ...state,
        triggerLoad: false,
        isLoading: false,
        isError: true,
        error: "",
      };
    default:
      throw new Error();
  }
}

// MAIN APP
/******************************************************************************/
const App = () => {
  const [svcStatuses, dispatchSvcStatuses] = React.useReducer(
    svcStatusesReducer,
    {
      data: [],
      triggerLoad: true,
      isLoading: false,
      isError: false,
      error: "",
    }
  );

  // CONVERT SERVICE STATUS FROM API TO `svcStatuses`
  function propsToHost(hostsFromApi) {
    // Initiate tmpStatuses
    const tmpStatuses = [];
    for (let svc of hostsFromApi) {
      if (!tmpStatuses.some((host) => host.hostname === svc.hostname)) {
        tmpStatuses.push({
          hostname: svc.hostname,
          services: [
            {
              name: svc.name,
              description: svc.description,
              enabled: svc.enabled ? "enabled" : "disabled",
              active_status: svc.active_status,
              last_check: svc.last_check,
            },
          ],
        });
      } else {
        // NOTE: I believe that, for objects, `Array.find()` returns a
        // reference
        let host = tmpStatuses.find((host) => host.hostname === svc.hostname);
        host.services.push({
          name: svc.name,
          description: svc.description,
          enabled: svc.enabled ? "enabled" : "disabled",
          active_status: svc.active_status,
          last_check: svc.last_check,
        });
      }
    }
    return tmpStatuses;
  }

  // LOAD SERVICE STATUSES
  /****************************************************************************/
  async function loadStatuses() {
    dispatchSvcStatuses({
      type: "STATUSES_FETCH_INIT",
    });

    try {
      await fetch(API_ALL_STATUSES)
        .then((response) => response.json())
        .then((result) => {
          console.log(result.data);
          dispatchSvcStatuses({
            type: "STATUSES_FETCH_SUCCESS",
            payload: propsToHost(result.data),
          });
        });
    } catch (err) {
      dispatchSvcStatuses({ type: "STATUSES_FETCH_FAILURE", payload: err });
    }
  }

  // INITIATE PAGE, INITIATE `hosts`
  /****************************************************************************/
  React.useEffect(() => {
    async function fetchData() {
      if (svcStatuses.triggerLoad) {
        await loadStatuses();
      }
    }
    fetchData();
  }, [svcStatuses.data]);

  document.title = "Service Monitor";

  return (
    <>
      <SVHeader />
      {svcStatuses.isError && <p>Something went wrong</p>}

      {svcStatuses.isLoading ? (
        <p>Loading...</p>
      ) : (
        <Hosts statuses={svcStatuses.data} doLoadStatuses={loadStatuses} />
      )}
    </>
  );
};

export default App;
