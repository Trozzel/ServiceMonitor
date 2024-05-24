import * as React from "react";
import uniqid from "uniqid";
import "bootstrap/dist/css/bootstrap.css";

import Service from "./Service";

// HELPER FUNCTIONS
/******************************************************************************/

// ALL HOSTS
/******************************************************************************/
const Hosts = ({ statuses, doLoadStatuses }) => {
  return (
    <>
      {statuses.map((status) => {
        return (
          <Host
            key={uniqid()}
            hostname={status.hostname}
            services={status.services}
            doLoadStatuses={doLoadStatuses}
          />
        );
      })}
    </>
  );
};

// HOST
/******************************************************************************/
const Host = ({ hostname, services, doLoadStatuses }) => (
  <>
    <div className="container bg-light my-4 pb-3 pt-2">
      <h2 className="text-center">{hostname}</h2>
      <div className="row g-3">
        {services.map((service) => {
          const serviceId = uniqid();
          return (
            <div className="col-4">
              <Service
                key={serviceId}
                hostname={hostname}
                service={service}
                doLoadStatuses={doLoadStatuses}
              />
            </div>
          );
        })}
      </div>
    </div>
  </>
);

export default Hosts;
