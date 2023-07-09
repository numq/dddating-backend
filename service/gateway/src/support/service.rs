use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::support::pb::{CreateTicketRequest, CreateTicketResponse, DeleteTicketRequest, DeleteTicketResponse, GetTicketByIdRequest, GetTicketByIdResponse, GetTicketsRequest, GetTicketsResponse, UpdateTicketRequest, UpdateTicketResponse};
use crate::support::pb::support_service_client::SupportServiceClient;
use crate::support::pb::support_service_server::SupportService;

pub struct SupportServiceImpl {
    client: SupportServiceClient<Channel>,
}

impl SupportServiceImpl {
    pub fn new(client: SupportServiceClient<Channel>) -> impl SupportService {
        SupportServiceImpl { client }
    }
}

#[tonic::async_trait]
impl SupportService for SupportServiceImpl {
    async fn get_tickets(&self, request: Request<GetTicketsRequest>) -> Result<Response<GetTicketsResponse>, Status> {
        self.client.clone().get_tickets(request).await
    }

    async fn get_ticket_by_id(&self, request: Request<GetTicketByIdRequest>) -> Result<Response<GetTicketByIdResponse>, Status> {
        self.client.clone().get_ticket_by_id(request).await
    }

    async fn create_ticket(&self, request: Request<CreateTicketRequest>) -> Result<Response<CreateTicketResponse>, Status> {
        self.client.clone().create_ticket(request).await
    }

    async fn update_ticket(&self, request: Request<UpdateTicketRequest>) -> Result<Response<UpdateTicketResponse>, Status> {
        self.client.clone().update_ticket(request).await
    }

    async fn delete_ticket(&self, request: Request<DeleteTicketRequest>) -> Result<Response<DeleteTicketResponse>, Status> {
        self.client.clone().delete_ticket(request).await
    }
}