using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace dotnet.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class Controller : ControllerBase
    {
        private readonly ILogger<Controller> _logger;

        public Controller(ILogger<Controller> logger)
        {
            _logger = logger;
        }

        [HttpGet()]
        [Route("/text")]
        public string Text()
        {
            return "value";
        }

        [HttpGet()]
        [Route("/json")]
        public object json()
        {
            return new { value = "value" };
        }

        [HttpPost()]
        [Route("/body")]
        public object json([FromBody] object body)
        {
            return body;
        }
    }
}
