// pages/about.jsx

import React from "react";

const About = () => {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "centre",
        alignItems: "centre",
        height: "100vh",
      }}
    >
      <h1>Service Monitor</h1>
      <p>
        Service Monitor is a simple way to add, view, and diagnose services
        running on Linux hosts
      </p>
    </div>
  );
};

export default About;
