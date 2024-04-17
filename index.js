import { html, render, useEffect, useState } from 'https://esm.sh/htm/preact/standalone'

function StaticSysinfo(props) {
    return html`
        <h2>Static Sysinfo</h2>
        <ul>
            <li>Operating System: ${props.static_info[0]} ${props.static_info[2]}</li>
            <li>Kernel Version: ${props.static_info[1]}</li>
            <li>Host Name: ${props.static_info[3]}</li>
        </ul>
    `;
}

function CPUsage(props) {
    return html`
        <h2>CPU Info</h2>
        <p>Number of cores: ${props.cores}</p>
        <p>Usage</p>
        <ul>
            ${props.cpu_usage.sort((a, b) => { return a - b }).map(usage => (
                html`<li>${usage}</li>`
            ))}
        </ul>
    `;
}

function App() {
    const [static_info, setStaticinfo] = useState([]);
    const [cpu_usage, setCPUusage] = useState([]);
    const [cores, setCores] = useState([]);

    useEffect(() => {
        fetch('/api/static_sysinfo')
            .then((res) => {
                return res.json();
            })
            .then((data) => {
                setStaticinfo(data);
            });

        fetch('/api/cpu_usage')
            .then((res) => {
                return res.json();
            })
            .then((data) => {
                setCPUusage(data);
            });

        fetch('/api/cores')
            .then((res) => {
                return res.json();
            })
            .then((data) => {
                console.log(data);
                setCores(data);
            });
    }, []);

    return html`
        <h1>Sysinfo Display</h1>
        <${StaticSysinfo} static_info=${static_info}/>
        <${CPUsage} cpu_usage=${cpu_usage} cores=${cores}/>
    `;
}

render(html`<${App} />`, document.body);
