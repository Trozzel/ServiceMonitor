import Button from "react-bootstrap/Button";
import Modal from "react-bootstrap/Modal";

import * as React from "react";

const Confirm = ({
  btnName,
  title,
  body,
  callbackFn,
  btnVariant = "primary",
}) => {
  // STATE FOR CLOSING MODAL
  const [show, setShow] = React.useState(false);

  const handleClose = () => setShow(false);
  const handleShow = () => {
    setShow(true);
  };

  const onSubmit = () => {
    callbackFn();
    setShow(false);
  };

  return (
    <>
      <Button variant={btnVariant} size="sm" onClick={handleShow}>
        {btnName}
      </Button>

      <Modal show={show} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>{title}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <p>{body}</p>
        </Modal.Body>
        <Modal.Footer>
          <div className="d-flex justify-content-end">
            <Button variant="primary" onClick={onSubmit}>
              Confirm
            </Button>
            <Button variant="secondary" onClick={handleClose}>
              Close
            </Button>
          </div>
        </Modal.Footer>
      </Modal>
    </>
  );
};

export default Confirm;
