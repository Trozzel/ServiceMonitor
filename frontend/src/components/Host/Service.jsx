import * as React from "react";
import axios from "axios";
import uniqid from "uniqid";
import "bootstrap/dist/css/bootstrap.css";

import {
  Modal,
  Container,
  Row,
  Col,
  Button,
  Tab,
  Tabs,
  Card,
  ListGroup,
} from "react-bootstrap";

import URL from "../helper";
import Confirm from "../Confirm";

// REMOVE SERVICE API
/******************************************************************************/
const URL_STATUS = `${URL}/api/systemctl/status?host=`;
const URL_SHOW = `${URL}/api/systemctl/show?host=`;
const URL_REM_SVC = `${URL}/api/hosts/remove_service`;

// REMOVE SERVICE REQUEST
/******************************************************************************/
async function removeService(hostname, service, loadStatusesCallback) {
  try {
    const response = await axios.patch(URL_REM_SVC, {
      hostname: hostname,
      service: service,
    });
    loadStatusesCallback();
    return response.data;
  } catch (error) {
    return error;
  }
}

// Service Details MODAL
/******************************************************************************/
const ServiceDetailsModal = ({ hostname, service }) => {
  // STATE FOR CLOSING MODAL
  const [show, setShow] = React.useState(false);

  const handleClose = () => setShow(false);
  const handleShow = () => {
    setIsLoading(true);
    loadDetails();
    setShow(true);
  };

  // FUNCTIONALITY FOR FETCHING SERVICE STATUS
  let details = new Map();
  const [detailsList, setDetailsList] = React.useState([]);

  const [isLoading, setIsLoading] = React.useState(true);

  async function loadDetails() {
    const service_url = `${URL_SHOW}${hostname}&service=${service}`;
    try {
      await fetch(service_url)
        .then((response) => response.json())
        .then((result) => {
          details = result.data;
          const list = [];
          for (const key in details) {
            if (details.hasOwnProperty(key)) {
              list.push(`${key}: ${details[key]}`);
            }
          }
          console.log("detailsList", detailsList);
          setDetailsList(list.sort());
          setIsLoading(false);
        });
    } catch (err) {
      details = err.data;
    }
  }

  return (
    <>
      <Button className="btn-sm mx-2" variant="primary" onClick={handleShow}>
        Details
      </Button>

      <Modal size="lg" show={show} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>
            {service}
            <h5 className="text-muted">{hostname}</h5>
          </Modal.Title>
        </Modal.Header>
        <Modal.Body scrollable="true">
          {isLoading ? (
            <p>Loading...</p>
          ) : (
            <ListGroup>
              {detailsList.map((line) => (
                <ListGroup.Item>
                  <code>{line}</code>
                </ListGroup.Item>
              ))}
            </ListGroup>
          )}
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleClose}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
};

// Service MODAL
/******************************************************************************/
const ServiceModal = ({ hostname, service }) => {
  // STATE FOR CLOSING MODAL
  const [show, setShow] = React.useState(false);

  const handleClose = () => setShow(false);
  const handleShow = () => {
    setIsLoading(true);
    loadStatus();
    setShow(true);
  };

  // FUNCTIONALITY FOR FETCHING SERVICE STATUS
  const [status, setStatus] = React.useState([]);
  const [isLoading, setIsLoading] = React.useState(true);

  async function loadStatus() {
    const service_url = `${URL_STATUS}${hostname}&service=${service}`;
    try {
      const response = await fetch(service_url);
      const result = await response.json();
      setStatus(result.data);
      setIsLoading(false);
    } catch (err) {
      setStatus([err.data]);
    }
  }

  return (
    <>
      <Button className="btn-sm mx-2" variant="primary" onClick={handleShow}>
        Status
      </Button>

      <Modal size="lg" show={show} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>
            {service}
            <h5 className="text-muted">{hostname}</h5>
          </Modal.Title>
        </Modal.Header>
        <Modal.Body scrollable="true">
          {isLoading ? (
            <p>Loading...</p>
          ) : (
            <ListGroup>
              {status.map((line) => (
                <ListGroup.Item>
                  <code>{line}</code>
                </ListGroup.Item>
              ))}
            </ListGroup>
          )}
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleClose}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
};

// SERVICE COMPONENT
/******************************************************************************/
const Service = ({ hostname, service, doLoadStatuses }) => {
  // START, STOP, RESTART SERVICE
  /****************************************************************************/
  async function doServiceAction(hostname, service, action) {
    const url = `${URL}/api/systemctl/${action}?host=${hostname}&service=${service}`;

    // TODO: handle no output from server. Look in stdout and stderr.
    try {
      await fetch(url)
        .then((response) => response.json())
        .then((result) => console.log(result.data));
    } catch (error) {
      console.log(error.data);
    }
  }

  function cardBorderStyle(service) {
    if (service.active_status !== "active") {
      return "danger";
    }
    return "";
  }

  return (
    <Tabs defaultActiveKey="status" id={uniqid()} className="mb-3">
      <Tab eventKey="status" title="Status">
        <Card className="text-center" border={cardBorderStyle(service)}>
          <Card.Header className="text-muted">
            {service.active_status}
          </Card.Header>
          <Card.Body>
            <Card.Title>{service.name}</Card.Title>
            <Card.Text>{service.description}</Card.Text>
            <ServiceModal hostname={hostname} service={service.name} />
            <ServiceDetailsModal
              hostname={hostname}
              service={service.name}
            ></ServiceDetailsModal>
            <Confirm
              key={uniqid()}
              btnName="Unfollow"
              title="Confirm unfollow"
              body={`Stop following, ${service.name}, on ${hostname}?`}
              callbackFn={() =>
                removeService(hostname, service.name, doLoadStatuses)
              }
              btnVariant="secondary"
            ></Confirm>
          </Card.Body>
        </Card>
      </Tab>
      <Tab eventKey="actions" title="Actions">
        <Card className="text-center">
          <Card.Header className="text-muted">{hostname}</Card.Header>
          <Card.Body>
            <Card.Title>{service.name}</Card.Title>
            <Card.Text>Actions for {service.name}</Card.Text>

            <Container>
              <Row>
                <Col>
                  <Confirm
                    key={uniqid()}
                    btnName="Start"
                    title="Confirm service start"
                    body={`Start ${service.name} on ${hostname}?`}
                    callbackFn={() =>
                      doServiceAction(hostname, service.name, "start")
                    }
                  ></Confirm>
                </Col>
                <Col>
                  <Confirm
                    key={uniqid()}
                    btnName="Restart"
                    title="Confirm service restart"
                    body={`Restart ${service.name} on ${hostname}?`}
                    callbackFn={() =>
                      doServiceAction(hostname, service.name, "restart")
                    }
                    btnVariant="primary"
                  ></Confirm>
                </Col>
                <Col>
                  <Confirm
                    key={uniqid()}
                    btnName="Stop"
                    title="Confirm stopping service?"
                    body={`Stop ${service.name} on ${hostname}?`}
                    callbackFn={() =>
                      doServiceAction(hostname, service.name, "stop")
                    }
                    btnVariant="secondary"
                  ></Confirm>
                </Col>
              </Row>
            </Container>
          </Card.Body>
        </Card>
      </Tab>
    </Tabs>
  );
};

// SERVICE COMPONENT
/******************************************************************************/
export default Service;
